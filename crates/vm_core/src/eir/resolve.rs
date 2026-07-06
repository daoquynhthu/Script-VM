//! Wire-to-closed-schema resolution for EIR modules.

use crate::eir::schema::{
    EirBlock, EirFunction, EirModule, EirOp, EirTerminator,
};
use crate::eir::validate::EirValidationError;
use crate::eir::wire::{
    validate_op_kind_tag, validate_terminator_kind_tag, EirBlockWire, EirModuleWire,
};

pub(crate) fn resolve_wire_module(wire: &EirModuleWire) -> Result<EirModule, EirValidationError> {
    let mut functions = Vec::with_capacity(wire.functions.len());
    for function in &wire.functions {
        let mut blocks = Vec::with_capacity(function.blocks.len());
        for block in &function.blocks {
            let mut ops = Vec::with_capacity(block.ops.len());
            for op in &block.ops {
                validate_op_kind_tag(op.kind_tag)?;
                let Some(kind) = op.kind.clone() else {
                    return Err(EirValidationError::UnknownOpKind(op.kind_tag));
                };
                ops.push(EirOp {
                    metadata: op.metadata.clone(),
                    kind,
                });
            }
            let terminator = resolve_block_terminator(block)?;
            blocks.push(EirBlock {
                block_id: block.block_id,
                parameters: block.parameters.clone(),
                ops,
                terminator,
                source_span: block.source_span,
            });
        }
        functions.push(EirFunction {
            eir_function_id: function.eir_function_id,
            function_id: function.function_id,
            module_id: function.module_id,
            entry_block: function.entry_block,
            blocks,
            slot_layout: function.slot_layout,
            frame_map: function.frame_map,
            source_span: function.source_span,
        });
    }
    Ok(EirModule {
        eir_version: wire.eir_version,
        source_runtime_plan_digest: wire.source_runtime_plan_digest,
        functions,
        constants: wire.constants.clone(),
        source_map: wire.source_map.clone(),
        root_maps: wire.root_maps.clone(),
        safepoints: wire.safepoints.clone(),
        deopt_points: wire.deopt_points.clone(),
    })
}

fn resolve_block_terminator(block: &EirBlockWire) -> Result<EirTerminator, EirValidationError> {
    if let Some(tag) = block.terminator_kind_tag {
        validate_terminator_kind_tag(tag)?;
    }
    block
        .terminator
        .clone()
        .ok_or(EirValidationError::BlockWithoutTerminator(block.block_id))
}